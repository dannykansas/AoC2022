# read input from file
with open('calories.txt') as f:
    data = f.read().splitlines()

# initialize variables
max_calories = 0
elf_calories = 0

# loop through the list of food Calories
for i in range(len(data)):
    # if the line is empty, reset the current Elf's Calories
    if data[i] == '':
        if elf_calories > max_calories:
            max_calories = elf_calories
        elf_calories = 0
    else:
        # add the current line's Calories to the current Elf's total
        elf_calories += int(data[i])

# check the last Elf's Calories
if elf_calories > max_calories:
    max_calories = elf_calories

# print the result
print("The Elf carrying the most Calories is carrying", max_calories, "Calories.")

