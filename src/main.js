const { invoke } = window.__TAURI__.tauri;

// --- DOM Elements ---
const directoryPathInput = document.getElementById('directory-path');
const browseBtn = document.getElementById('browse-btn');
const ignoreItemsTextarea = document.getElementById('ignore-items');
const generateBtn = document.getElementById('generate-btn');
const outputTree = document.getElementById('output-tree');
const copyBtn = document.getElementById('copy-btn');
const clearBtn = document.getElementById('clear-btn');

const initialMessage = "Select a directory and click 'Generate' to see the tree.";

// --- Functions ---

/**
 * Loads initial data from the backend.
 */
async function loadInitialData() {
    try {
        // Simplified to only get the ignore list
        const defaultIgnoreList = await invoke('get_initial_data');
        ignoreItemsTextarea.value = defaultIgnoreList.join('\n');
    } catch (error) {
        console.error('Failed to load initial data:', error);
        outputTree.textContent = `Error: ${error}`;
    }
}

/**
 * Handles the directory selection process and updates the input field.
 */
async function selectDirectory() {
    try {
        const selectedPath = await invoke('select_directory');
        if (selectedPath) {
            directoryPathInput.value = selectedPath;
        }
    } catch (error) {
        console.error('Error selecting directory:', error);
        outputTree.textContent = `Error: ${error}`;
    }
}

/**
 * Calls the backend to generate the folder tree.
 */
async function generateTree() {
    const path = directoryPathInput.value;
    if (!path) {
        alert('Please select a directory first.');
        return;
    }

    const ignoreList = ignoreItemsTextarea.value
        .split('\n')
        .map(item => item.trim())
        .filter(item => item.length > 0);

    outputTree.textContent = 'Generating...';
    generateBtn.disabled = true;

    try {
        const tree = await invoke('generate_tree', { path, ignoreList });
        outputTree.textContent = tree;
    } catch (error) {
        outputTree.textContent = `An error occurred:\n${error}`;
    } finally {
        generateBtn.disabled = false;
    }
}

/**
 * Copies the generated tree to the clipboard.
 */
function copyToClipboard() {
    navigator.clipboard.writeText(outputTree.textContent)
        .then(() => {
            copyBtn.textContent = 'Copied!';
            setTimeout(() => { copyBtn.textContent = 'Copy to Clipboard'; }, 2000);
        })
        .catch(err => console.error('Failed to copy:', err));
}

/**
 * Clears the output panel.
 */
function clearOutput() {
    outputTree.textContent = initialMessage;
}


// --- Event Listeners ---
browseBtn.addEventListener('click', selectDirectory);
generateBtn.addEventListener('click', generateTree);
copyBtn.addEventListener('click', copyToClipboard);
clearBtn.addEventListener('click', clearOutput);

// --- Initialization ---
document.addEventListener('DOMContentLoaded', loadInitialData);