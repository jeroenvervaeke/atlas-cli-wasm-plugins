// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
#include "plugin.h"
#include <stdlib.h>
#include <string.h>

// Imported Functions from `atlascli:plugin/host@0.0.1`

__attribute__((__import_module__("atlascli:plugin/host@0.0.1"), __import_name__("bearer-token")))
extern void __wasm_import_atlascli_plugin_host_bearer_token(uint8_t *);

// Exported Functions from `atlascli:plugin/info@0.0.1`

__attribute__((__weak__, __export_name__("cabi_post_atlascli:plugin/info@0.0.1#name")))
void __wasm_export_exports_atlascli_plugin_info_name_post_return(uint8_t * arg0) {
  if ((*((size_t*) (arg0 + 4))) > 0) {
    free(*((uint8_t **) (arg0 + 0)));
  }
}

__attribute__((__weak__, __export_name__("cabi_post_atlascli:plugin/info@0.0.1#sub-commands")))
void __wasm_export_exports_atlascli_plugin_info_sub_commands_post_return(uint8_t * arg0) {
  size_t len = *((size_t*) (arg0 + 4));
  if (len > 0) {
    uint8_t *ptr = *((uint8_t **) (arg0 + 0));
    for (size_t i = 0; i < len; i++) {
      uint8_t *base = ptr + i * 8;
      (void) base;
      if ((*((size_t*) (base + 4))) > 0) {
        free(*((uint8_t **) (base + 0)));
      }
    }
    free(ptr);
  }
}

__attribute__((__weak__, __export_name__("cabi_post_atlascli:plugin/info@0.0.1#run")))
void __wasm_export_exports_atlascli_plugin_info_run_post_return(uint8_t * arg0) {
  switch ((int32_t) (int32_t) *((uint8_t*) (arg0 + 0))) {
    case 0: {
      break;
    }
    case 1: {
      if ((*((size_t*) (arg0 + 8))) > 0) {
        free(*((uint8_t **) (arg0 + 4)));
      }
      break;
    }
  }
}

// Canonical ABI intrinsics

__attribute__((__weak__, __export_name__("cabi_realloc")))
void *cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size) {
  (void) old_size;
  if (new_size == 0) return (void*) align;
  void *ret = realloc(ptr, new_size);
  if (!ret) abort();
  return ret;
}

// Helper Functions

void plugin_list_string_free(plugin_list_string_t *ptr) {
  size_t list_len = ptr->len;
  if (list_len > 0) {
    plugin_string_t *list_ptr = ptr->ptr;
    for (size_t i = 0; i < list_len; i++) {
      plugin_string_free(&list_ptr[i]);
    }
    free(list_ptr);
  }
}

void exports_atlascli_plugin_info_result_void_string_free(exports_atlascli_plugin_info_result_void_string_t *ptr) {
  if (!ptr->is_err) {
  } else {
    plugin_string_free(&ptr->val.err);
  }
}

void plugin_string_set(plugin_string_t *ret, const char*s) {
  ret->ptr = (uint8_t*) s;
  ret->len = strlen(s);
}

void plugin_string_dup(plugin_string_t *ret, const char*s) {
  ret->len = strlen(s);
  ret->ptr = (uint8_t*) cabi_realloc(NULL, 0, 1, ret->len * 1);
  memcpy(ret->ptr, s, ret->len * 1);
}

void plugin_string_free(plugin_string_t *ret) {
  if (ret->len > 0) {
    free(ret->ptr);
  }
  ret->ptr = NULL;
  ret->len = 0;
}

// Component Adapters

__attribute__((__aligned__(4)))
static uint8_t RET_AREA[12];

void atlascli_plugin_host_bearer_token(plugin_string_t *ret) {
  __attribute__((__aligned__(4)))
  uint8_t ret_area[8];
  uint8_t *ptr = (uint8_t *) &ret_area;
  __wasm_import_atlascli_plugin_host_bearer_token(ptr);
  *ret = (plugin_string_t) { (uint8_t*)(*((uint8_t **) (ptr + 0))), (*((size_t*) (ptr + 4))) };
}

__attribute__((__export_name__("atlascli:plugin/info@0.0.1#name")))
uint8_t * __wasm_export_exports_atlascli_plugin_info_name(void) {
  plugin_string_t ret;
  exports_atlascli_plugin_info_name(&ret);
  uint8_t *ptr = (uint8_t *) &RET_AREA;
  *((size_t*)(ptr + 4)) = (ret).len;
  *((uint8_t **)(ptr + 0)) = (uint8_t *) (ret).ptr;
  return ptr;
}

__attribute__((__export_name__("atlascli:plugin/info@0.0.1#sub-commands")))
uint8_t * __wasm_export_exports_atlascli_plugin_info_sub_commands(void) {
  plugin_list_string_t ret;
  exports_atlascli_plugin_info_sub_commands(&ret);
  uint8_t *ptr = (uint8_t *) &RET_AREA;
  *((size_t*)(ptr + 4)) = (ret).len;
  *((uint8_t **)(ptr + 0)) = (uint8_t *) (ret).ptr;
  return ptr;
}

__attribute__((__export_name__("atlascli:plugin/info@0.0.1#run")))
uint8_t * __wasm_export_exports_atlascli_plugin_info_run(void) {
  exports_atlascli_plugin_info_result_void_string_t ret;
  exports_atlascli_plugin_info_run(&ret);
  uint8_t *ptr = (uint8_t *) &RET_AREA;
  if ((ret).is_err) {
    const plugin_string_t *payload0 = &(ret).val.err;*((int8_t*)(ptr + 0)) = 1;
    *((size_t*)(ptr + 8)) = (*payload0).len;
    *((uint8_t **)(ptr + 4)) = (uint8_t *) (*payload0).ptr;
  } else {
    *((int8_t*)(ptr + 0)) = 0;
  }
  return ptr;
}

// Ensure that the *_component_type.o object is linked in

extern void __component_type_object_force_link_plugin(void);
void __component_type_object_force_link_plugin_public_use_in_this_compilation_unit(void) {
  __component_type_object_force_link_plugin();
}
