# Typographic scale

> #### [The typographic scale](https://spencermortensen.com/articles/typographic-scale/)
>
> The third and final property of any scale is its *fundamental frequency*, $f_0$. In the chromatic scale, this is the Stuttgart pitch. In the classic typographic scale, the fundamental frequency is the *pica*. This value, 1 *pica* = 12 pt, is the baseline font size used in print typography.
>
> And here is the formula for the frequency $f_i$ of the $i$th note in the scale:
> $$
> f_i = f_0 r^{\frac{1}{n}}
> $$
> Using this formula, we can calculate every font size in the classic typographic scale:
>
> ![](Classic typographical scale.svg)
>
> Classic typographical scale.

The scale in use here is a “golden tetratonic” scale, where $f_0 = 10$, $r = \phi$ and $n = 5$., rather than the classic typographical scale.[^phi]

[^phi]: where $\phi$ is the [golden ratio](https://en.wikipedia.org/wiki/Golden_ratio): 1.618033988749….



For each of the eight accessibility options there are eleven standard type sizes; numbered 0–10.


|  $i$   | XXS  |  XS  |  S   |  M   |  L   |  XL  | XXL  | XXXL |
| :----: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| **0**  |  10  |  11  |  11  |  12  |  12  |  14  |  16  |  18  |
| **1**  |  11  |  12  |  13  |  14  |  15  |  17  |  19  |  21  |
| **2**  |  12  |  13  |  14  |  15  |  16  |  18  |  20  |  22  |
| **3**  |  13  |  14  |  15  |  16  |  17  |  19  |  21  |  23  |
| **4**  |  15  |  16  |  17  |  18  |  19  |  21  |  23  |  25  |
| **5**  |  16  |  17  |  18  |  19  |  20  |  22  |  24  |  26  |
| **6**  |  18  |  19  |  20  |  21  |  22  |  24  |  26  |  28  |
| **7**  |  20  |  22  |  24  |  26  |  28  |  30  |  32  |  34  |
| **8**  |  22  |  24  |  26  |  28  |  30  |  32  |  34  |  36  |
| **9**  |  24  |  26  |  28  |  30  |  32  |  34  |  36  |  38  |
| **10** |  26  |  28  |  30  |  32  |  34  |  36  |  38  |  40  |



The changes in font size differ; both across the accessibility adjustments and down the standard type sizes


|  $i$   | XXS  |  XS  |  S   |  M   |  L   |  XL  | XXL  | XXXL |
| :----: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| **0**  |  10  |  +1  |  +0  |  +1  |  +0  |  +2  |  +2  |  +2  |
| **1**  |  11  |  +1  |  +1  |  +1  |  +1  |  +2  |  +2  |  +2  |
| **2**  |  12  |  +1  |  +1  |  +1  |  +1  |  +2  |  +2  |  +2  |
| **3**  |  13  |  +1  |  +1  |  +1  |  +1  |  +2  |  +2  |  +2  |
| **4**  |  15  |  +1  |  +1  |  +1  |  +1  |  +2  |  +2  |  +2  |
| **5**  |  16  |  +1  |  +1  |  +1  |  +1  |  +2  |  +2  |  +2  |
| **6**  |  18  |  +1  |  +1  |  +1  |  +1  |  +2  |  +2  |  +2  |
| **7**  |  20  |  +2  |  +2  |  +2  |  +2  |  +2  |  +2  |  +2  |
| **8**  |  22  |  +2  |  +2  |  +2  |  +2  |  +2  |  +2  |  +2  |
| **9**  |  24  |  +2  |  +2  |  +2  |  +2  |  +2  |  +2  |  +2  |
| **10** |  26  |  +2  |  +2  |  +2  |  +2  |  +2  |  +2  |  +2  |



## Apple Typography Comparison

Apple platforms have their own [typography schemes](https://developer.apple.com/design/human-interface-guidelines/typography). 



### macOS

Assuming that `.AppleSystemUIFont` and `.SFNS-Regular` resolve to the same font:

| forTextStyle: | wght    | pt.  | spc  | NSFont                               |
| :------------ | ------- | :--: | :--: | ------------------------------------ |
| .largeTitle   | regular | 26.0 | 5.73 |                                      |
| .title1       | regular | 22.0 | 5.05 |                                      |
| .title2       | regular | 17.0 | 4.35 |                                      |
| .title3       | regular | 15.0 | 3.98 |                                      |
| .headline     | bold    | 13.0 | 3.28 | .titleBarFont                        |
| .body         | regular | 13.0 | 3.58 | .systemFont, .messageFont, .menuFont |
| .callout      | regular | 12.0 | 3.38 | .controlContentFont                  |
| .subheadline  | regular | 11.0 | 3.16 | .paletteFont, .toolTipsFont          |
| .footnote     | regular | 10.0 | 2.93 |                                      |
| .caption1     | regular | 10.0 | 2.93 | .labelFont                           |
| .caption2     | medium  | 10.0 | 2.84 |                                      |

#### Notes:

1. The names of a few of the NSFont keys are misleading:
   - [labelFont(ofSize fontSize: CGFloat)](https://developer.apple.com/documentation/appkit/nsfont/1528213-labelfont)  
     The label font […] is used for the labels on toolbar buttons and to label tick marks on full-size sliders
   - [messageFont(ofSize fontSize: CGFloat)](https://developer.apple.com/documentation/appkit/nsfont/1525777-messagefont)  
     Returns the font used for standard interface items, such as button labels, menu items, and so on, in the specified size.  
     
     `.messageFont` is actually the standard font for labels on controls. 
2. The effects of [NSControl.ControlSize](https://developer.apple.com/documentation/appkit/nscontrol/controlsize) are not shown here.
3. Neither are the user fonts:
   - `userFont(ofSize fontSize: CGFloat)`, and
   - `userFixedPitchFont(ofSize fontSize: CGFloat)`  
   
   Which are **Helvetica 12.0 pt., spc=3.33** and **Menlo-Regular 11.0 pt., spc=6.62**, respectively.

4. The [macOS built-in text styles](https://developer.apple.com/design/human-interface-guidelines/typography#macOS-built-in-text-styles) lists bolder font weights than simple testing revealed.



### iOS

The iOS [xSmall](https://developer.apple.com/design/human-interface-guidelines/typography#xSmall) [iOS Dynamic Type sizes](https://developer.apple.com/design/human-interface-guidelines/typography#iOS-iPadOS-Dynamic-Type-sizes) is the closest match to the macOS sizes shown above, but it is larger is every category.

#### xSmall

| forTextStyle: | wght      | size | leading | difference |
| :------------ | :-------- | :--: | :-----: | :--------- |
| .largeTitle   | regular   | 31.0 |  38.0   | +5         |
| .title1       | regular   | 25.0 |  31.0   | +3         |
| .title2       | regular   | 19.0 |  24.0   | +2         |
| .title3       | regular   | 17.0 |  22.0   | +2         |
| .headline     | semi-bold | 14.0 |  19.0   | +1         |
| .body         | regular   | 14.0 |  19.0   | +1         |
| .callout      | regular   | 13.0 |  18.0   | +1         |
| .subheadline  | regular   | 12.0 |  16.0   | +1         |
| .footnote     | regular   | 12.0 |  16.0   | +2         |
| .caption1     | regular   | 11.0 |  13.0   | +1         |
| .caption2     | regular   | 11.0 |  13.0   | +1         |

Similar comparisons can be done down the rest of the iOS sizes.

#### Small

| forTextStyle: | wght      | size | leading | difference |
| :------------ | :-------- | :--: | :-----: | :--------- |
| .largeTitle   | regular   | 32.0 |  39.0   | +1         |
| .title1       | regular   | 26.0 |  32.0   | +1         |
| .title2       | regular   | 20.0 |  25.0   | +1         |
| .title3       | regular   | 18.0 |  23.0   | +1         |
| .headline     | semi-bold | 15.0 |  20.0   | +1         |
| .body         | regular   | 15.0 |  20.0   | +1         |
| .callout      | regular   | 14.0 |  19.0   | +1         |
| .subheadline  | regular   | 13.0 |  18.0   | +1/+2      |
| .footnote     | regular   | 12.0 |  16.0   | —          |
| .caption1     | regular   | 11.0 |  13.0   | —          |
| .caption2     | regular   | 11.0 |  13.0   | —          |



#### Medium

| forTextStyle: | wght      | size | leading | difference |
| :------------ | :-------- | :--: | :-----: | :--------- |
| .largeTitle   | regular   | 33.0 |  40.0   | +1         |
| .title1       | regular   | 27.0 |  33.0   | +1         |
| .title2       | regular   | 21.0 |  26.0   | +1         |
| .title3       | regular   | 19.0 |  24.0   | +1         |
| .headline     | semi-bold | 16.0 |  21.0   | +1         |
| .body         | regular   | 16.0 |  21.0   | +1         |
| .callout      | regular   | 15.0 |  20.0   | +1         |
| .subheadline  | regular   | 14.0 |  19.0   | +1         |
| .footnote     | regular   | 12.0 |  16.0   | —          |
| .caption1     | regular   | 11.0 |  13.0   | —          |
| .caption2     | regular   | 11.0 |  13.0   | —          |



#### Large

This is the default iOS size for Dynamic Type.

| forTextStyle: | wght      | size | leading | difference |
| :------------ | :-------- | :--: | :-----: | :--------- |
| .largeTitle   | regular   | 34.0 |  41.0   | +1         |
| .title1       | regular   | 28.0 |  34.0   | +1         |
| .title2       | regular   | 22.0 |  28.0   | +1/+4      |
| .title3       | regular   | 20.0 |  25.0   | +1         |
| .headline     | semi-bold | 17.0 |  22.0   | +1         |
| .body         | regular   | 17.0 |  22.0   | +1         |
| .callout      | regular   | 16.0 |  21.0   | +1         |
| .subheadline  | regular   | 15.0 |  20.0   | +1         |
| .footnote     | regular   | 13.0 |  18.0   | +1/+2      |
| .caption1     | regular   | 12.0 |  16.0   | +1/+3      |
| .caption2     | regular   | 11.0 |  13.0   | —          |



#### xLarge

| forTextStyle: | wght      | size | leading | difference |
| :------------ | :-------- | :--: | :-----: | :--------- |
| .largeTitle   | regular   | 36.0 |  43.0   | +2         |
| .title1       | regular   | 30.0 |  37.0   | +2/+3      |
| .title2       | regular   | 24.0 |  30.0   | +2         |
| .title3       | regular   | 22.0 |  28.0   | +2/+3      |
| .headline     | semi-bold | 19.0 |  24.0   | +2         |
| .body         | regular   | 19.0 |  24.0   | +2         |
| .callout      | regular   | 18.0 |  23.0   | +2         |
| .subheadline  | regular   | 17.0 |  22.0   | +2         |
| .footnote     | regular   | 15.0 |  20.0   | +2         |
| .caption1     | regular   | 14.0 |  19.0   | +2/+3      |
| .caption2     | regular   | 13.0 |  18.0   | +2/+5      |



#### xxLarge

| forTextStyle: | wght      | size | leading | difference |
| :------------ | :-------- | :--: | :-----: | :--------- |
| .largeTitle   | regular   | 38.0 |  46.0   | +2/+3      |
| .title1       | regular   | 32.0 |  39.0   | +2         |
| .title2       | regular   | 26.0 |  32.0   | +2         |
| .title3       | regular   | 24.0 |  30.0   | +2         |
| .headline     | semi-bold | 21.0 |  26.0   | +2         |
| .body         | regular   | 21.0 |  26.0   | +2         |
| .callout      | regular   | 20.0 |  25.0   | +2         |
| .subheadline  | regular   | 19.0 |  24.0   | +2         |
| .footnote     | regular   | 17.0 |  22.0   | +2         |
| .caption1     | regular   | 16.0 |  21.0   | +2         |
| .caption2     | regular   | 15.0 |  20.0   | +2         |



#### xxxLarge

| forTextStyle: | wght      | size | leading | difference |
| :------------ | :-------- | :--: | :-----: | :--------- |
| .largeTitle   | regular   | 40.0 |  48.0   | +2         |
| .title1       | regular   | 34.0 |  41.0   | +2         |
| .title2       | regular   | 28.0 |  34.0   | +2         |
| .title3       | regular   | 26.0 |  32.0   | +2         |
| .headline     | semi-bold | 23.0 |  29.0   | +2/+3      |
| .body         | regular   | 23.0 |  29.0   | +2/+3      |
| .callout      | regular   | 22.0 |  28.0   | +2/+3      |
| .subheadline  | regular   | 21.0 |  28.0   | +2/+4      |
| .footnote     | regular   | 19.0 |  24.0   | +2         |
| .caption1     | regular   | 18.0 |  23.0   | +2         |
| .caption2     | regular   | 17.0 |  22.0   | +2         |



## Material Typography Comparison



### Material 3

| Material | style  | size |         forTextStyle: | NSFont                               |
| :------- | :----: | :--: | --------------------: | :----------------------------------- |
| Display  | large  |  57  | .extraExtraLargeTitle |                                      |
|          | medium |  45  |      .extraLargeTitle |                                      |
|          | small  |  36  |                       |                                      |
| Headline | large  |  32  |           .largeTitle |                                      |
|          | medium |  28  |               .title1 |                                      |
|          | small  |  24  |               .title2 |                                      |
| Title    | large  |  22  |               .title3 |                                      |
|          | medium |  16  |             .headline | .titleBarFont                        |
|          | small  |  14  |          .subheadline | .paletteFont, .toolTipsFont          |
| Body     | large  |  16  |                 .body | .systemFont, .messageFont, .menuFont |
|          | medium |  14  |              .callout | .controlContentFont                  |
|          | small  |  12  |                       |                                      |
| Label    | large  |  14  |             ,footnote |                                      |
|          | medium |  12  |             .caption1 | .labelFont                           |
|          | small  |  11  |             .caption2 |                                      |




# Design Language

| $i$  | XXS  | XS   | L    | XL   |
| ---- | ---- | ---- | ---- | ---- |
| 1    | 10   | 11   | 12   | 14   |
| 2    | 11   | 12   | 15   | 17   |
| 3    | 12   | 13   | 16   | 18   |
| 4    | 13   | 14   | 17   | 19   |
| 5    | 15   | 16   | 19   | 21   |
| 6    | 16   | 17   | 20   | 22   |
| 7    | 18   | 19   | 22   | 24   |
| 8    | 20   | 22   | 28   | 30   |
| 9    | 22   | 24   | 30   | 32   |
| 10   | 24   | 26   | 32   | 34   |
| 11   | 26   | 28   | 34   | 36   |



[TOC]
