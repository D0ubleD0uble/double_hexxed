
import init, { set_tile, set_show_tile_labels } from "./pkg/double_hexxed.js"
run();

async function run() {
    try {
        await init();
    } catch (error) {
        if (
            error instanceof Error &&
            error.message.includes("Using exceptions for control flow")
        ) {
            console.warn("Ignoring wasm-bindgen control flow exception:", error.message);
        } else {
            throw error; // Unexpected error
        }
    }
    
    // Custom UI logic
    // TILES must match asset_loading "AssetTag"
    const TILES_BY_GROUP = {
        "Simple": [
            "Delete",
            "Outline",
            "Blank (white)",
            "Lush (green)",
            "Ocean (blue)",
            "Rocky (gray)",
            "Snowy (off-white)"
        ],
        "Forests": [
            "Sparse Trees, Lush",
            "Sparse Trees, Snowy",
            "Forest Conifer, Lush",
            "Forest Conifer, Snowy",
            "Forest Deciduous, Lush",
            "Forest Mixed, Lush"
        ],
        "Hills": [
            "Hills, Desert",
            "Hills, Lush",
            "Hills, Snowy",
            "Mountain Foothills, Lush",
            "Mountain Foothills, Rocky",
            "Mountain Foothills, Snowy"
        ],
        "Mountains": [
            "Mountain Volcano, Lush",
            "Mountain Volcano, Rocky",
            "Mountain Volcano, Snowy",
            "Mountain Low, Lush",
            "Mountain Low, Rocky",
            "Mountain Low, Snowy",
            "Mountain Medium, Lush",
            "Mountain Medium, Rocky",
            "Mountain Medium, Snowy",
            "Mountain Peak, Lush",
            "Mountain Peak, Rocky",
            "Mountain Peak, Snowy"
        ],
        "Plains": [
            "Plains, Damp",
            "Plains, Desert",
            "Plains, Farmland",
            "Plains, Lush"
        ],
        "Snow": [
            "Snow area",
            "Snow drifts",
            "Snow field"
        ],
        "Urban": [
            "Ruins, Desert",
            "Ruins, Lush",
            "Urban City, Lush",
            "Urban Farm, Lush",
            "Urban Farmland, Lush",
            "Urban Town, Abandoned",
            "Urban Town, Inhabited",
            "Urban Town Lumberyard",
            "Urban Monastery",
            "Urban Tower",
            "Urban Town, Lush"
        ],
        "Water": [
            "Ocean Soft Waves",
            "Ocean Still Water",
            "Ocean Waves",
            "Wetlands, Damp"
        ]
    };
    const toolbox = document.getElementById("toolbox");
    
    Object.entries(TILES_BY_GROUP).forEach(([groupName, tiles]) => {
        const group = document.createElement("div");
        group.className = "tile-group";
                
        const header = document.createElement("h4");
        header.textContent = `${groupName} ▸`;
        header.onclick = () => toggleGroup(header);
        group.appendChild(header);
                
        const tileList = document.createElement("div");
        tileList.className = "tile-list";
                
        tiles.forEach(tileName => {
            const button = document.createElement("button");
            button.className = "tile-button";
            button.textContent = tileName;
            button.addEventListener("click", () => {
                set_tile(tileName);
            });
            tileList.appendChild(button);
        });
    
        group.appendChild(tileList);
        toolbox.appendChild(group);
    });
    
    const checkbox = document.getElementById("toggle-labels-checkbox");
    checkbox.addEventListener("change", () => {
        set_show_tile_labels(checkbox.checked);
    });
}