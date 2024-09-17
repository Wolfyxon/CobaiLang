public const PI: num = 3.14159265359

public func abs(number: num) num:
        if n < 0:
            return -n
        end

        return n
end

public func clamp(value: num, min: num, max: num) num:
    if value < min:
        return min
    end

    if value > max:
        return max
    end
    
    return n
end