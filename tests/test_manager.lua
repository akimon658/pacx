---@param pkg string
local function subcmd_test(pkg)
  print(pkg)
end

---@param pkg string
---@param flags string
local function subcmd_test_with_arg_and_flags(pkg, flags)
  print(string.format("%s %s", pkg, flags))
end

---@param flags string
local function subcmd_test_with_flags(_, flags)
  print(flags)
end

return {
  subcmd_test = subcmd_test,
  subcmd_test_with_arg_and_flags = subcmd_test_with_arg_and_flags,
  subcmd_test_with_flags = subcmd_test_with_flags,
}
