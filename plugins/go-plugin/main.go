package main

import (
	"fmt"

	. "go.plugin.example/gen"
)

type PluginImpl struct {
}

func (p PluginImpl) Name() string {
	return "go-plugin"
}

func (p PluginImpl) SubCommands() []string {
	return []string{"go"}
}

func (p PluginImpl) Run() Result[struct{}, string] {
	fmt.Println("Hello from go!")
	return Ok[struct{}, string](struct{}{})
}

func init() {
	plugin := PluginImpl{}
	SetExportsAtlascliPlugin0_0_1_Info(plugin)
}

func main() {}
