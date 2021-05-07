# Burnside calculator

This is a repo for http://burnside-calc.com/, a tool for enumerating distinct combinations, using Burnside's Lemma.

## What is Burnside's Lemma?

Here's a handwavy, unmathematical description:

You have a collection of things, and ways that those things can be transformed into each other. These transformations follow certain rules:

1. You can always *not* transform the thing.
2. For every transformation, there's another valid transformation that will revert it.
3. If A and B are valid transformations, then applying A then B is also a valid transformation.

That last one means that if you're allowed to rotate by 90 degrees, then you also have to be able to rotate by 180 degrees.

For each of these transformations, you can compute how many things in your collection are unchanged by that transformation. For example, if you're working with 2×2 tiles, with each quadrant coloured one of three different colours, then the only tiles that aren't transformed by a 90° rotate are those where the quadrents are all the same colour: the north-west corner has to match the north-east corner it will be rotated to. Similarly, the north-east corner has to match the south-east corner, which needs to match the south-west corner, which needs to match the north-west corner.

By contrast, for a 180° rotation, only opposite corners need to match, giving us 3×3 = 9 tiles unaffected by a 180° rotation.

In total, there are 3⁴ tiles unaffacted by a 0° rotation; 3 unaffected by a 90° rotation; the same for a 270° rotation, and 9 unaffected by a 180° rotation.

Burnside's lemma states that if you divide this total by the number of transformations (4), you get the total number of *distinct* tiles.

(81 + 3 + 3 + 9) / 4 = 96 / 4 = 24 distinct 2×2 tiles with three colours. 
