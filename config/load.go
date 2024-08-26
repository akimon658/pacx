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
	Info     lua.LFunction
	Install  lua.LFunction
	List     lua.LFunction
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

var ErrFunctionNotDefined = errors.New("function not defined")

func (c *Config) Info(pkgName string) error {
	if err := c.callLuaFunc(&c.raw.Info, lua.LString(pkgName)); err != nil {
		return fmt.Errorf("failed to execute function info: %w", err)
	}
	return nil
}

func (c *Config) Install(pkgName string) error {
	if err := c.callLuaFunc(&c.raw.Install, lua.LString(pkgName)); err != nil {
		return fmt.Errorf("failed to execute function install: %w", err)
	}
	return nil
}

func (c *Config) List() error {
	if err := c.callLuaFunc(&c.raw.List); err != nil {
		return fmt.Errorf("failed to execute function list: %w", err)
	}
	return nil
}

func (c *Config) Outdated() error {
	if err := c.callLuaFunc(&c.raw.Outdated); err != nil {
		return fmt.Errorf("failed to execute function outdated: %w", err)
	}
	return nil
}

func (c *Config) Upgrade(pkgName string) error {
	if err := c.callLuaFunc(&c.raw.Upgrade, lua.LString(pkgName)); err != nil {
		return fmt.Errorf("failed to execute function upgrade: %w", err)
	}
	return nil
}

func (c *Config) callLuaFunc(fn *lua.LFunction, args ...lua.LValue) error {
	if fn.Proto == nil {
		return ErrFunctionNotDefined
	}

	co, _ := c.luaState.NewThread()
	_, err, _ := c.luaState.Resume(co, fn, args...)

	return err
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
