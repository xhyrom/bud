-- handle prompts, etc
local name = get_prompt(1)
local like_me = get_prompt(2)

print("Hello, " .. name .. "!")

if like_me == "yes" then
  print("I like you too!")
else
  print("I'm sorry to hear that.")
end

local file = io.open("data.txt", "w")
if file == nil then
  print("Error: Could not open file")
  return
end

file:write("Name: " .. name .. "\n" .. "Like me: " .. like_me .. "\n")