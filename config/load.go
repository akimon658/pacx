package config

import (
	"fmt"
	"path/filepath"

	"github.com/adrg/xdg"
	"github.com/yuin/gluamapper"
	lua "github.com/yuin/gopher-lua"
)

type rawConfig struct {
	Outdated lua.LFunction
	Upgrade  lua.LFunction
}

type Config struct {
	luaState *lua.LState
	raw      rawConfig
}

func (c *Config) Close() {
	c.luaState.Close()
}

func (c *Config) Outdated() error {
	co, _ := c.luaState.NewThread()
	_, err, _ := c.luaState.Resume(co, &c.raw.Outdated)
	if err != nil {
		return fmt.Errorf("failed to execute function outdated: %w", err)
	}

	return nil
}

func (c *Config) Upgrade(pkgs ...string) error {
	pkgsLuaString := make([]lua.LValue, len(pkgs))
	for i := range pkgs {
		pkgsLuaString[i] = lua.LString(pkgs[i])
	}

	co, _ := c.luaState.NewThread()
	_, err, _ := c.luaState.Resume(co, &c.raw.Upgrade, pkgsLuaString...)
	if err != nil {
		return fmt.Errorf("failed to execute function upgrade: %w", err)
	}

	return nil
}

func Load(pkgManager string) (*Config, error) {
	var cfg Config
	cfg.luaState = lua.NewState()

	if err := cfg.luaState.DoFile(filepath.Join(xdg.ConfigHome, "pacx", pkgManager+".lua")); err != nil {
		return nil, fmt.Errorf("failed to open config file: %w", err)
	}

	var rawCfg rawConfig
	ret := cfg.luaState.Get(-1)
	if err := gluamapper.Map(ret.(*lua.LTable), &rawCfg); err != nil {
		return nil, fmt.Errorf("failed to load configuration: %w", err)
	}

	cfg.raw = rawCfg

	return &cfg, nil
}
