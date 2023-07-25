Feature: Spheres

  Scenario: A ray intersects a sphere at two points
    Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    And s ← sphere()
    When xs ← intersect(s, r)
    Then xs.count = 2
    And xs[0] = 4.0
    And xs[1] = 6.0

  Scenario: A ray intersects a sphere at a tangent
    Given r ← ray(point(0, 1, -5), vector(0, 0, 1))
    And s ← sphere()
    When xs ← intersect(s, r)
    Then xs.count = 2
    And xs[0] = 5.0
    And xs[1] = 5.0

  Scenario: A ray misses a sphere
    Given r ← ray(point(0, 2, -5), vector(0, 0, 1))
    And s ← sphere()
    When xs ← intersect(s, r)
    Then xs.count = 0

  Scenario: A ray originates inside a sphere
    Given r ← ray(point(0, 0, 0), vector(0, 0, 1))
    And s ← sphere()
    When xs ← intersect(s, r)
    Then xs.count = 2
    And xs[0] = -1.0
    And xs[1] = 1.0

  Scenario: A sphere is behind a ray
    Given r ← ray(point(0, 0, 5), vector(0, 0, 1))
    And s ← sphere()
    When xs ← intersect(s, r)
    Then xs.count = 2
    And xs[0] = -6.0
    And xs[1] = -4.0

  Scenario: Intersect sets the object on the intersection
    Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    And s ← sphere()
    When xs ← intersect(s, r)
    Then xs.count = 2
  # TODO object equality doesn't really work with trait objects
  # And xs[0].object = s
  # And xs[1].object = s

  Scenario: A sphere's default transformation
    Given s ← sphere()
    Then s.transform = identity_matrix

  Scenario: Creating a sphere with a transformation
    Given t ← translation(2, 3, 4)
    And s ← sphere(t)
    Then s.transform = t

  Scenario: Intersecting a scaled sphere with a ray
    Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    And t ← scaling(2, 2, 2)
    And s ← sphere(t)
    And xs ← intersect(s, r)
    Then xs.count = 2
    And xs[0].t = 3
    And xs[1].t = 7

  Scenario: Intersecting a translated sphere with a ray
    Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    And t ← translation(5, 0, 0)
    And s ← sphere(t)
    And xs ← intersect(s, r)
    Then xs.count = 0

  Scenario: The normal on a sphere at a point on the x axis
    Given s ← sphere()
    When n ← normal_at(s, point(1, 0, 0))
    Then n = vector(1, 0, 0)

  Scenario: The normal on a sphere at a point on the y axis
    Given s ← sphere()
    When n ← normal_at(s, point(0, 1, 0))
    Then n = vector(0, 1, 0)

  Scenario: The normal on a sphere at a point on the z axis
    Given s ← sphere()
    When n ← normal_at(s, point(0, 0, 1))
    Then n = vector(0, 0, 1)

  Scenario: The normal on a sphere at a nonaxial point
    Given s ← sphere()
    # When n ← normal_at(s, point(√3/3, √3/3, √3/3))
    When n ← normal_at(s, point(0.577350269, 0.577350269, 0.577350269))
    # Then n = vector(√3/3, √3/3, √3/3)
    Then n = vector(0.577350269, 0.577350269, 0.577350269)

  Scenario: The normal is a normalized vector
    Given s ← sphere()
    #When n ← normal_at(s, point(√3/3, √3/3, √3/3))
    When n ← normal_at(s, point(0.577350269, 0.577350269, 0.577350269))
    Then n = normalize(n)

  Scenario: Computing the normal on a translated sphere
    Given t ← translation(0, 1, 0)
    And s ← sphere(t)
    When n ← normal_at(s, point(0, 1.70711, -0.70711))
    Then n = vector(0, 0.70711, -0.70711)

  Scenario: Computing the normal on a transformed sphere
    # Given m ← scaling(1, 0.5, 1) * rotation_z(π/5)
    Given m ← scaling(1, 0.5, 1) * rotation_z(0.628318530)
    And s ← sphere(m)
    # When n ← normal_at(s, point(0, √2/2, -√2/2))
    When n ← normal_at(s, point(0, 0.707106781, -0.707106781))
    Then n = vector(0, 0.97014, -0.24254)

  Scenario: A sphere has a default material
    Given s ← sphere()
    When m ← s.material
    Then m = material()

  Scenario: A sphere may be assigned a material
    Given s ← sphere()
    And m ← material()
    And m.ambient ← 1
    When s.material ← m
    Then s.material = m

  Scenario: A helper for producing a sphere with a glassy material
    Given s ← glass_sphere()
    Then s.transform = identity_matrix
    And s.material.transparency = 1.0
    And s.material.refractive_index = 1.5
