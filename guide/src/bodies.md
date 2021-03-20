# Bodies

Heron supports multiple types of _bodies_, which define the
collision shape of the entities.

## `Body::Sphere`

### 3d

A (sphere)[https://en.wikipedia.org/wiki/Sphere].

It is defined by its `radius`.

Example::

```rust
Body::Sphere {
    radius: 20.0
};
```

### 2d

A (circle)[https://en.wikipedia.org/wiki/Circle].

Example::

```rust
Body::Sphere {
    radius: 20.0
};
```

## `Body::Cuboid`

### 3d

A (rectangular cuboid)[https://en.wikipedia.org/wiki/Cuboid], i.e. a box with
all right angles.

It is defined by a `half_extends` vector which is half of its
Example:

```rust
Body::Cuboid {
    half_extends: Vec3::new(5.0, 10.0, 7.0)
};
```

### 2d

A (rectangle)[https://en.wikipedia.org/wiki/Rectangle].

## `Body::Capsule`

In 3d, a (capsule)[https://en.wikipedia.org/wiki/Capsule_(geometry)] is a
cylinder with hemispherical ends; think a capsule you can swallow.

In 2d this is a stadium - an intersection of the 3d capsule. This shape is
known as a (stadium)[https://en.wikipedia.org/wiki/Stadium_(geometry)].

## `Body::ConvexHull`
