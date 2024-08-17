package config

import (
	"errors"
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
	if c.raw.Outdated.Proto == nil {
		return errors.New("function outdated not defined")
	}

	co, _ := c.luaState.NewThread()
	_, err, _ := c.luaState.Resume(co, &c.raw.Outdated)
	if err != nil {
		return fmt.Errorf("failed to execute function outdated: %w", err)
	}

	return nil
}

func (c *Config) Upgrade(pkgName string) error {
	if c.raw.Upgrade.Proto == nil {
		return errors.New("function upgrade not defined")
	}

	co, _ := c.luaState.NewThread()
	_, err, _ := c.luaState.Resume(co, &c.raw.Upgrade, lua.LString(pkgName))
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

	ret := cfg.luaState.Get(-1)
	if err := gluamapper.Map(ret.(*lua.LTable), &cfg.raw); err != nil {
		return nil, fmt.Errorf("failed to load configuration: %w", err)
	}

	return &cfg, nil
}
