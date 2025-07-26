
import init, { set_tool, set_show_tile_labels } from "./pkg/double_hexxed.js"
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
    // TOOLS must match asset_loading "AssetTag"
    const TOOLS_BY_CATEGORY = {
        "Simple": [
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
    
    Object.entries(TOOLS_BY_CATEGORY).forEach(([categoryName, tools]) => {
        const category = document.createElement("div");
        category.className = "tool-category";
                
        const header = document.createElement("h4");
        header.textContent = `${categoryName} ▸`;
        header.onclick = () => toggleCategory(header);
        category.appendChild(header);
                
        const toolList = document.createElement("div");
        toolList.className = "tool-list";
                
        tools.forEach(toolName => {
            const button = document.createElement("button");
            button.textContent = toolName;
            button.addEventListener("click", () => {
                set_tool(toolName);
            });
            toolList.appendChild(button);
        });
    
        category.appendChild(toolList);
        toolbox.appendChild(category);
    });
    
    const checkbox = document.getElementById("toggle-labels-checkbox");
    checkbox.addEventListener("change", () => {
        set_show_tile_labels(checkbox.checked);
    });
}