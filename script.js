
document.body.addEventListener("contextmenu", (e) => {
    e.preventDefault();
    e.stopPropagation();
});

// Insert hack to make sound autoplay on Chrome as soon as the user interacts with the tab:
// https://developers.google.com/web/updates/2018/11/web-audio-autoplay#moving-forward

// the following function keeps track of all AudioContexts and resumes them on the first user
// interaction with the page. If the function is called and all contexts are already running,
// it will remove itself from all event listeners.
(function () {
    // An array of all contexts to resume on the page
    const audioContextList = [];

    // An array of various user interaction events we should listen for
    const userInputEventNames = [
        "click",
        "contextmenu",
        "auxclick",
        "dblclick",
        "mousedown",
        "mouseup",
        "pointerup",
        "touchend",
        "keydown",
        "keyup",
    ];

    // A proxy object to intercept AudioContexts and
    // add them to the array for tracking and resuming later
    self.AudioContext = new Proxy(self.AudioContext, {
        construct(target, args) {
            const result = new target(...args);
            audioContextList.push(result);
            return result;
        },
    });

    // To resume all AudioContexts being tracked
    function resumeAllContexts(_event) {
        let count = 0;

        audioContextList.forEach((context) => {
            if (context.state !== "running") {
                context.resume();
            } else {
                count++;
            }
        });

        // If all the AudioContexts have now resumed then we unbind all
        // the event listeners from the page to prevent unnecessary resume attempts
        // Checking count > 0 ensures that the user interaction happens AFTER the game started up
        if (count > 0 && count === audioContextList.length) {
            userInputEventNames.forEach((eventName) => {
                document.removeEventListener(eventName, resumeAllContexts);
            });
        }
    }

    // We bind the resume function for each user interaction
    // event on the page
    userInputEventNames.forEach((eventName) => {
        document.addEventListener(eventName, resumeAllContexts);
    });
})();

const helpButton = document.getElementById("help-button");
const controlsPanel = document.getElementById("controls");

helpButton.addEventListener("click", () => {
    if (controlsPanel.style.display === "none") {
        controlsPanel.style.display = "block";
    } else {
        controlsPanel.style.display = "none";
    }
});

function toggleGroup(header) {
    const group = header.parentElement;
    group.classList.toggle('open');
}

document.getElementById("alpha-close").addEventListener("click", () => {
    document.getElementById("alpha-disclaimer").style.display = "none";
});

document.getElementById('tile-filter').addEventListener('input', (event) => {
  const query = event.target.value.trim().toLowerCase();
  const buttons = document.querySelectorAll('.tile-button');

  buttons.forEach(button => {
    const label = button.textContent.toLowerCase();
    button.hidden = !label.includes(query);
  });
});

// ==================== MOBILE SUPPORT ====================

// Hamburger menu toggle
const toolboxToggle = document.getElementById('toolbox-toggle');
const toolbox = document.getElementById('toolbox');
const toolboxOverlay = document.getElementById('toolbox-overlay');

if (toolboxToggle) {
    toolboxToggle.addEventListener('click', () => {
        const isOpen = toolbox.classList.contains('open');

        if (isOpen) {
            closeToolbox();
        } else {
            openToolbox();
        }
    });
}

if (toolboxOverlay) {
    toolboxOverlay.addEventListener('click', closeToolbox);
}

function openToolbox() {
    toolbox.classList.add('open');
    toolboxToggle.classList.add('active');
    toolboxOverlay.classList.add('visible');
}

function closeToolbox() {
    toolbox.classList.remove('open');
    toolboxToggle.classList.remove('active');
    toolboxOverlay.classList.remove('visible');
}

// Close toolbox when a tile is selected on mobile
document.addEventListener('click', (e) => {
    if (e.target.classList.contains('tile-button') && window.innerWidth < 768) {
        closeToolbox();
    }
});

// Mobile zoom controls
const zoomInBtn = document.getElementById('zoom-in');
const zoomOutBtn = document.getElementById('zoom-out');

if (zoomInBtn) {
    zoomInBtn.addEventListener('click', () => {
        // Simulate mouse wheel scroll up (zoom in)
        const canvas = document.querySelector('canvas');
        if (canvas) {
            const wheelEvent = new WheelEvent('wheel', {
                deltaY: -100,
                clientX: canvas.width / 2,
                clientY: canvas.height / 2,
                bubbles: true
            });
            canvas.dispatchEvent(wheelEvent);
        }
    });
}

if (zoomOutBtn) {
    zoomOutBtn.addEventListener('click', () => {
        // Simulate mouse wheel scroll down (zoom out)
        const canvas = document.querySelector('canvas');
        if (canvas) {
            const wheelEvent = new WheelEvent('wheel', {
                deltaY: 100,
                clientX: canvas.width / 2,
                clientY: canvas.height / 2,
                bubbles: true
            });
            canvas.dispatchEvent(wheelEvent);
        }
    });
}

// Touch gesture support
let touchState = {
    startDistance: 0,
    lastDistance: 0,
    isPinching: false,
    lastX: 0,
    lastY: 0,
    startTime: 0,
    hasMoved: false
};

const canvas = document.querySelector('canvas');

if (canvas) {
    // Touch start
    canvas.addEventListener('touchstart', (e) => {
        touchState.startTime = Date.now();
        touchState.hasMoved = false;

        if (e.touches.length === 2) {
            // Two-finger gesture (pinch or pan)
            e.preventDefault();
            touchState.isPinching = true;

            const dx = e.touches[1].clientX - e.touches[0].clientX;
            const dy = e.touches[1].clientY - e.touches[0].clientY;
            touchState.startDistance = Math.sqrt(dx * dx + dy * dy);
            touchState.lastDistance = touchState.startDistance;

            // Store center point for panning
            touchState.lastX = (e.touches[0].clientX + e.touches[1].clientX) / 2;
            touchState.lastY = (e.touches[0].clientY + e.touches[1].clientY) / 2;
        } else if (e.touches.length === 1) {
            // Single touch - store for tap detection
            touchState.lastX = e.touches[0].clientX;
            touchState.lastY = e.touches[0].clientY;
        }
    }, { passive: false });

    // Touch move
    canvas.addEventListener('touchmove', (e) => {
        touchState.hasMoved = true;

        if (e.touches.length === 2) {
            e.preventDefault();

            // Calculate current distance for pinch zoom
            const dx = e.touches[1].clientX - e.touches[0].clientX;
            const dy = e.touches[1].clientY - e.touches[0].clientY;
            const currentDistance = Math.sqrt(dx * dx + dy * dy);

            // Pinch zoom
            if (touchState.isPinching) {
                const delta = currentDistance - touchState.lastDistance;
                const scaleFactor = 2; // Adjust sensitivity

                const wheelEvent = new WheelEvent('wheel', {
                    deltaY: -delta * scaleFactor,
                    clientX: (e.touches[0].clientX + e.touches[1].clientX) / 2,
                    clientY: (e.touches[0].clientY + e.touches[1].clientY) / 2,
                    bubbles: true
                });
                canvas.dispatchEvent(wheelEvent);

                touchState.lastDistance = currentDistance;
            }

            // Two-finger pan
            const centerX = (e.touches[0].clientX + e.touches[1].clientX) / 2;
            const centerY = (e.touches[0].clientY + e.touches[1].clientY) / 2;

            const deltaX = centerX - touchState.lastX;
            const deltaY = centerY - touchState.lastY;

            // Simulate arrow key presses based on pan direction
            if (Math.abs(deltaX) > 2 || Math.abs(deltaY) > 2) {
                // Use keyboard events to move camera
                if (Math.abs(deltaX) > Math.abs(deltaY)) {
                    const key = deltaX > 0 ? 'ArrowRight' : 'ArrowLeft';
                    const keyEvent = new KeyboardEvent('keydown', { key, bubbles: true });
                    document.dispatchEvent(keyEvent);
                } else {
                    const key = deltaY > 0 ? 'ArrowDown' : 'ArrowUp';
                    const keyEvent = new KeyboardEvent('keydown', { key, bubbles: true });
                    document.dispatchEvent(keyEvent);
                }

                touchState.lastX = centerX;
                touchState.lastY = centerY;
            }
        }
    }, { passive: false });

    // Touch end
    canvas.addEventListener('touchend', (e) => {
        const touchDuration = Date.now() - touchState.startTime;
        const movementThreshold = 10; // pixels

        // If it was a quick tap (not a drag), treat it as a click
        if (e.touches.length === 0 &&
            !touchState.hasMoved &&
            touchDuration < 300 &&
            e.changedTouches.length === 1) {

            const touch = e.changedTouches[0];

            // Create and dispatch a click event at the touch location
            const clickEvent = new MouseEvent('click', {
                clientX: touch.clientX,
                clientY: touch.clientY,
                bubbles: true,
                cancelable: true
            });
            canvas.dispatchEvent(clickEvent);
        }

        // Reset touch state
        if (e.touches.length < 2) {
            touchState.isPinching = false;
            touchState.startDistance = 0;
            touchState.lastDistance = 0;
        }
    });

    // Prevent context menu on long press
    canvas.addEventListener('contextmenu', (e) => {
        if (e.touches || touchState.hasMoved) {
            e.preventDefault();
        }
    }, { passive: false });
}

// Handle orientation changes
window.addEventListener('orientationchange', () => {
    // Close toolbox on orientation change to avoid UI issues
    closeToolbox();

    // Give the browser time to adjust layout
    setTimeout(() => {
        const canvas = document.querySelector('canvas');
        if (canvas) {
            // Force canvas to recalculate size
            const resizeEvent = new Event('resize');
            window.dispatchEvent(resizeEvent);
        }
    }, 200);
});