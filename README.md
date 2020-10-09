# pseudo_rust_cli
Pseudo CLI created to beat rust documentation chapter 8 last challenge.

# How to use
After running 'cargo run', the following commands are avaiable:

add [employee name] [company department name] \n
remove [employee name] [company department name]

The add command will create if not existent an entry in a Hashmap with the key equivalent to [company department name];
The expected output for any of the mentioned commands is the hashmaps's print;

# Example
add John Marketing

Output: 
{
  "Marketing": [
    "John"
  ]
}
