use core::panic;
use std::collections::BTreeMap;

lazy_static::lazy_static! {
    static ref OPENAPI_SPEC: oas3::OpenApiV3Spec = {
        let spec_bytes = include_bytes!("../atlas-api.yaml");
        oas3::from_reader(&spec_bytes[..]).unwrap()
    };

    pub static ref SPEC: Spec = {
        Spec::new(&OPENAPI_SPEC)
    };
}

pub const PREFIX: &str = "/api/atlas/v2";

#[derive(Debug)]
pub struct Spec {
    pub children: BTreeMap<String, Node>,
}

#[derive(Debug)]
pub enum Node {
    Entity(Entity),
    Group(Group),
}

#[derive(Debug)]
pub struct Entity {
    pub list: bool,
    pub delete_all: bool,
    pub create: bool,
    pub delete: bool,
    pub get: bool,
    pub update: bool,
    pub children: BTreeMap<String, Node>,
}

#[derive(Debug)]
pub struct Group {
    pub children: BTreeMap<String, Node>,
}

impl Spec {
    fn new(openapi_spec: &oas3::OpenApiV3Spec) -> Spec {
        let project_paths = openapi_spec
            .paths
            .iter()
            .filter(|(path, _item)| path.starts_with("/api/atlas/v2/groups/{groupId}"))
            .map(|(path, item)| {
                let mut api_path = ApiPath::from(path.as_str());
                api_path.segments.drain(..6);
                (api_path, item)
            })
            .collect::<BTreeMap<_, _>>();

        let mut entities = BTreeMap::new();

        // search for entities
        for (path, item) in &project_paths {
            // test if this a detail endpoint (ending on an ID)
            if path
                .segments
                .last()
                .map(|last| last.is_id())
                .unwrap_or_default()
            {
                let get = item.get.is_some();
                let delete = item.delete.is_some();
                let update = item.patch.is_some();

                if !(get || delete) {
                    continue;
                }

                // try to find the "all" endpoint
                let mut all_endpoint_path = path.clone();
                all_endpoint_path.segments.pop();

                let all_item = if let Some(all_item) = project_paths.get(&all_endpoint_path) {
                    all_item
                } else {
                    continue;
                };

                let list = all_item.get.is_some();
                let create = all_item.post.is_some();
                let delete_all = all_item.delete.is_some();

                if !list {
                    continue;
                }

                entities.insert(
                    all_endpoint_path,
                    RawEntity {
                        list,
                        delete_all,
                        create,
                        delete,
                        get,
                        update,
                    },
                );
            }
        }

        let mut root_node = RawNode::default();

        for (path, entity) in entities {
            let mut node = &mut root_node;

            for segment in path.segments {
                node = node.get_mut_child(segment);
            }

            node.entity = Some(entity)
        }

        Spec {
            children: match Node::from(root_node) {
                Node::Entity(_) => panic!(),
                Node::Group(g) => g.children,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiPath {
    segments: Vec<Segment>,
}

impl<'a> From<&'a str> for ApiPath {
    fn from(value: &'a str) -> Self {
        let mut segments = Vec::new();
        for segment in value.split('/') {
            if segment.starts_with('{') && segment.ends_with('}') {
                let segments_len = segment.chars().count();
                segments.push(Segment::Id(
                    segment.chars().skip(1).take(segments_len - 2).collect(),
                ));
            } else {
                segments.push(Segment::Const(segment.to_string()));
            }
        }

        ApiPath { segments }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Segment {
    Const(String),
    Id(String),
}

impl Segment {
    fn is_id(&self) -> bool {
        match self {
            Segment::Const(_) => false,
            Segment::Id(_) => true,
        }
    }
}

#[derive(Debug, Default)]
struct RawNode {
    entity: Option<RawEntity>,
    children: BTreeMap<Segment, RawNode>,
}

impl RawNode {
    fn get_mut_child(&mut self, segment: Segment) -> &mut RawNode {
        self.children.entry(segment).or_default()
    }
}

#[derive(Debug, Default)]
struct RawEntity {
    list: bool,
    delete_all: bool,
    create: bool,
    delete: bool,
    get: bool,
    update: bool,
}

impl From<RawNode> for Node {
    fn from(value: RawNode) -> Self {
        if let Some(entity) = value.entity {
            let children = value
                .children
                .into_iter()
                .filter(|(s, _)| s.is_id())
                .flat_map(|(_, i)| i.children)
                .filter_map(|(s, i)| match s {
                    Segment::Const(s) => Some((s, Node::from(i))),
                    Segment::Id(_) => None,
                })
                .collect();

            Node::Entity(Entity {
                list: entity.list,
                delete_all: entity.delete_all,
                create: entity.create,
                delete: entity.delete,
                get: entity.get,
                update: entity.update,
                children,
            })
        } else {
            let children = value
                .children
                .into_iter()
                .filter_map(|(s, i)| match s {
                    Segment::Const(s) => Some((s, Node::from(i))),
                    Segment::Id(_) => None,
                })
                .collect();

            Node::Group(Group { children })
        }
    }
}
