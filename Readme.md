# bevy_icon_creator

by Franklin E. Blanco

## Description

An icon creator for bevy. Take any entity, mesh, model and convert it into an icon/texture with ease! Supports creating multiple textures at once.

## Bevy Version

| `bevy_icon_creator` | `bevy` |
| :--                 | :--    |
| `0.1`               | `0.13.1/0.13` |

## Usage

Let's say you had an Item, lets call this item a Sword. You want a texture created for the sword. Maybe for your inventory system.

What you have to do is the following:

- Add the main plugin to your bevy app:

```rust
app.add_plugins(IconCreatorPlugin::default());
```

You can customize some parts of the plguin. Go to the `IconCreatorPlugin` for more information.

- Spawn the entity and any entities contained inside that entity. If the sword is comprised of a Sword parent entity (empty) and then two children: The handle & the Blade, you want to spawn it normally. You can either: Spawn the entity with a Visibility::Hidden (Don't worry, this plugin automatically sets it to Visibility::Visible after moving to the scene.) Or just spawn it somewhere not visible, far away. Either way, it will only be in the scene for a frame, so you could also not care about it.

"NOTE: Everything rendered in the scenes is placed recursively in a different RenderLayer."

- When spawned, mark the parent entity with `NeedsIconMarker`. The only required field for instantiating it is an Uuid. This will be with what you later find the image and use it. Use `NeedsIconMarker::new(uuid)`.
You can set some custom optional parameters, using `with_transform()` & `with_extra_frames()`.
Extra frames mean how many more frames you want the scene to wait to load the item & the transform basically means the added transform given to the entity after being placed in the scene.
Use this to offset the position, rotation or scale for the entity, so that the texture created shows.

- As soon as you mark the entity, it will be reparented to an available scene.

- Then, after 3 + (EXTRA_FRAMES) frames, the image will be available in the resource `CreatedIcons`. Call `get_image_handle(uuid)` with the uuid that you passed initially to the `NeedsIconMarker`. (The image should be available on the first you mark NeedsIconMarker, but it will be a blank one until it gets loaded.)

- I've included a more convenient way to work with this, you want to use the `SetImageOnLoadMarker(uuid)` and add it to any UiImage. When it's added it will poll the CreatedIcons resource until the image is rendered. Once it's there, it will automatically swap the texture for that UiImage.

Thanks for using!

This was extracted from my in development game: ## Operation Deep Freeze

[Subreddit](https://www.reddit.com/r/operationdeepfreeze/)

[Youtube channel](https://www.youtube.com/channel/UCaqu5sampSjdA8_mB55UiXQ)

If you have any inquiries: <me@franklinblanco.dev>
