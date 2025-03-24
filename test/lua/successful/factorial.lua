
function FACT(b, acc)
    if b < 2 then
        return acc
    else
        return FACT(b-1, acc * b)
    end
end

local function fact(b)
    if b < 2 then
        return 1
    else
        return b * fact(b-1)
    end
end

local function factorial(n)
    local acc = 1
    for i = 1, n, 1 do
        acc = acc * i
    end
    return acc
end

local N = 5

local function inv_factorial()
    local acc = 0
    for i = N, 1, -1 do
        acc = acc + factorial(i)
        local function print_acc()
            print(acc)
        end
        print_acc()
    end
    return acc
end

print(FACT(10, 1))
print(fact(5))
print(factorial(5))
print(inv_factorial())