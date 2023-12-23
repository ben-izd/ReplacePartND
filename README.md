# ReplacePartND
A Mathematica LibraryLink library to efficiently replace numerical data in place

## How to use
First, you have to change the path to the library and execute the following code:
```mathematica
ClearAll[ReplacePartNDInPlace];

ReplacePartNDInPlace = 
  LibraryFunctionLoad[
   "C:\\Users\\USER_NAME\\Downloads\\replace_part_nd.dll", 
   "replace_part_nd", {{_, _, "Shared"}, {Integer, 2, "Constant"}, {_, 1, "Constant"}}, "Void"];
```
Remember the changes are in place just like `list[[i]] = x`. You have to have the following inputs:
- Data could be any dimension (Integer or Real)
- Positions you want to change (it should be a 2D list like: `{{1}, {2}}` or `{{1,2}, {2,2}}`)
- Values (should be 1D list - Integer or Real - but the type must match the Data)

## Example
```mathematica
Block[{x = Range[10]},
 ReplacePartNDInPlace[x, {{2}, {5}, {10}}, {-1, -2, -3}];
 x
 ]
(* Output: {1, -1, 3, 4, -2, 6, 7, 8, 9, -3} *)
```

## Possible Issues
If you provide an invalid position like `{11}` for the case above the kernel will crash (Could be fix in the future).
