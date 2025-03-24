local function sum(n)
    local function bis(m, acc)
        if m < 1 then
            return acc
        end
        return bis(m-1, acc + m)
    end
    return bis(n, 0)
end

print(sum(250))
print(sum(1000))