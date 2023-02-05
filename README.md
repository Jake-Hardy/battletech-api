# Battletech API

The Battletech API is an attempt to create a convenient and comprehensive tool for creating and managing Battletech unit sheets. 

## Roadmap

- CRUD operations for 'Mech units, including:
	- :heavy_check_mark: 'Mech name and designation 
	- :heavy_check_mark: Components (e.g., Center Torso) and their internal structure and armor values 
	- :x: Equipment as a children of Components and their respective relevant stats (e.g., ammo)
	- :x: Stats (e.g., movement points, heat, tonnage)
- Instanced stat calculation, including:
	- :x: Heat build-up 
	- :x: Expenditure of ammo
	- :x: Inward movement of damage from destroyed parts
	- :x: Resolving attacks (making and receiving)

Depending on time and how this project works out, maybe functionality will be built out for other unit types (e.g., infantry, vehicles, turrets, protomechs, etc.).

## Running Locally

- Build the Docker container
	- `docker build -t battletech-api .`
- Run the Docker container
	- `docker run -it --rm --network=host battletech-api`