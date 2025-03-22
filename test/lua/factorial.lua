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

print(FACT(5, 1))
print(fact(5))