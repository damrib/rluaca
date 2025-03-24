function F(a)
    return a
end

local function f(b)
    return b
end

print(1 ~= 2)
print(true == true)
print("Hello" == "Hello")
print("1" ~= 1)
print(nil == nil)
print(print == print)
print(f ~= F)
print(F == F)
print(f == f)