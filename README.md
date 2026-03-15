The SDL bindings here follow proper lifetime rules, ensuring no use-after-free happens during compile time. Textures are constrained by Renderer which is constrained by the sdl initialization (which is an empty struct, but basically tells you that SDL has been initialized.

The way things currently stand, you will need to build sdl3 and sdl3_image and place it in your C libraries path. I've only tried this in Linux. APT doesn't have sdl3 yet, so I cloned it directly from github.
