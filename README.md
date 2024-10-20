# Performance issues for DOM update in Leptos
Demonstration showing performance issues in DOM updates for a Leptos app when a signal triggers many DOM changes.
For my machine, performance issues become noticable with 1000 elements.

## Controls
### Loop control
+ `Iter`: Iterates over elements.
+ `For`: Uses a `<For>` component to iterate over elements.

### Display
`Dynamic value` indicates whether the enetered value will be displayed or a static value is. This was used to determine whther it was the DOM updates causing the issues, or reading the signal.