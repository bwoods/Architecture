# Typographic scales



> #### [The typographic scale](https://spencermortensen.com/articles/typographic-scale/)
>
> The third and final property of any scale is its *fundamental frequency*, $f_0$. In the chromatic scale, this is the Stuttgart pitch. In the classic typographic scale, the fundamental frequency is the *pica*. This value, 1 *pica* = 12 pt, is the baseline font size used in print typography.
>
> And here is the formula for the frequency $f_i$ of the ith note in the scale:
> $$
> f_i = f_0 r^{\frac{1}{n}}
> $$
> Using this formula, we can calculate every font size in the classic typographic scale:
>
> ![](Classic typographical scale.svg)
>
> Classic typographical scale.



## Apple Typography Examples



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

##### Note:

The names of a few of the NSFont keys are misleading:

- `messageFont(ofSize fontSize: CGFloat)`  
    Returns the font used for standard interface items, such as button labels, menu items, and so on, in the specified size.
- `labelFont(ofSize fontSize: CGFloat)`  
    The label font […] is used for the labels on toolbar buttons and to label tick marks on full-size sliders

`.messageFont` is the standard font for labels on controls. 

Also note that the effects of [NSControl.ControlSize](https://developer.apple.com/documentation/appkit/nscontrol/controlsize) have not been examined yet.



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

If the macOS typography is re-branded **xxSmall**, a similar comparison can be done down the rest of the iOS sizes.

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



