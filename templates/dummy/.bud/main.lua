/* handle prompts, etc */
local name = get_prompt(1)
local like_me = get_prompt(2)

print("Hello, " .. name .. "!")

if like_me == "yes" then
  print("I like you too!")
else
  print("I'm sorry to hear that.")
end