# Weapon implementation
On fire, a few things will happen.
If ranged, a bullet will fire that will:
1. reference the weapon using Rc<RefCell<Weapon>>
2. will launch until the bullet hits something.
3. If it hits a character, it will damage using the information on the weapon

Very temporary, we will use a JSON file to store the weapon data
