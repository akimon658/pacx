---@param pkg string
---@param flags string
---@param version? string
local function subcmd_test(pkg, flags, version)
  print(string.format("pkg: %s, flags: %s, version: %s", pkg, flags, version or "nil"))
end

---@param pkg string
---@param flags string
---@param version? string
local function subcmd_test_with_arg_and_flags(pkg, flags, version)
  print(string.format("pkg: %s, flags: %s, version: %s", pkg, flags, version or "nil"))
end

---@param pkg string
---@param flags string
---@param version? string
local function subcmd_test_with_flags(pkg, flags, version)
  print(string.format("flags: %s, version: %s (pkg was: %s)", flags, version or "nil", pkg or "nil"))
end

return {
  subcmd_test = subcmd_test,
  subcmd_test_with_arg_and_flags = subcmd_test_with_arg_and_flags,
  subcmd_test_with_flags = subcmd_test_with_flags,
}
