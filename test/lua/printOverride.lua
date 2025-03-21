NEW_PRINT = print
print = 2

local local_print = NEW_PRINT
NEW_PRINT(print)
local_print(NEW_PRINT)