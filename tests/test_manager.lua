---@param pkg string
local function subcmd_test(pkg)
  print(pkg)
end

return {
  subcmd_test = subcmd_test,
}
