local a = 42
local function g(b)
	local d = 1 + b
	local e = 1 + d
	local g = 1 + e
	return d + e + b + g
end
local b = g(a)
print(b)
