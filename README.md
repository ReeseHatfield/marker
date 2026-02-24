# marker
Generate markdown documentation from typst doc comments

## Problem:
- Typst has no first-party support for doc comments
- Typst Universe requires you to have a good documentation in your README.md
- Tidy only renders to typst and *not* markdown


## Solution
Marker generates markdown documentation from typst function doc comments.
It adopts a similar (albeit hacky) syntax to Javadoc.
It exports your doc comments to markdown for more maintainable typst documentation.

## Syntax

### Header
All doc comments lines are created with three leading `///` above a function signature.
The first line should contain the function name, a colon and a space, followed by the description.
Any lines before the first parameter is also treated as the description.
```java
/// multiple_choice: Create a multiple choice question
/// This function will render directly to the page ...
```

### Parameters
Parameters begin with the `@param` tag after the leading `///`.
After the param, you add the name of the parameter, and the type, followed by the description, separated by spaces.
```java
/// @param body content Body of question
```
If your parameter has a default argument, it should be placed as `= {value}` immediately after the parameter type.
```java
/// @param points int = 1 Points the question is worth
```

If your parameter can have multiple (unioned) types, place the types within square brackets `[ ]`s.
```java
/// @param cols [int | array ] = 1 Number of columns to render the answer. Pass an array of units for specific spacing e.g. (1fr, 1fr, 12pt)
```
An example of a full (non returning) doc comment can be found below
```java
/// multiple_choice: Create a multiple choice question
/// This function will render directly to the page ...
/// @param body content Body of question
/// @param points int = 1 Points the question is worth
/// @param cols [int | array ] = 1 Number of columns to render the answer. Pass an array of units for specific spacing e.g. (1fr, 1fr, 12pt)
#let multiple_choice(body, points: 1, cols: 1, ..answers) = { ... }
```

### Returns
Returns are denoted with an `@return` following  the `///`.
After the `@return` tag, you say the data type of the return, followed by the description
```java
/// @return array Array of num fr units
```
The return tag can be omitted if the function returns no value, and instead renders directly to the document.
A full example of a function that returns a value can be found below.

```java
/// _num_to_fr_units: Map a number into a tuple of 1fr units
/// primarily used to make optional column passing to #multiple_choice easier
/// input = 3 -> output = (1fr, 1fr, 1fr)
/// input = 5 -> output = (1fr, 1fr, 1fr, 1fr, 1fr)
/// @param num int number to map
/// @return array Array of num fr units
#let _num_to_fr_units(num) = {
    range(num).map(i => 1fr)
}
```
