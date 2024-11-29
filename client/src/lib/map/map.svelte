<script lang="ts">
    const MAP_SIZE = 64;
    let scale = 1;
    let offsetX = 0;
    let offsetY = 0;
    let isDragging = false;
    let startX = 0;
    let startY = 0;

    const tiles = Array(MAP_SIZE).fill(null).map(() => 
        Array(MAP_SIZE).fill(null).map(() => ({
            type: 'grass', // default tile type
            // Add other tile properties as needed
        }))
    );

    function handleWheel(event: WheelEvent) {
        event.preventDefault();
        const delta = event.deltaY > 0 ? 0.9 : 1.1;
        scale = Math.max(0.2, Math.min(3, scale * delta));
    }

    function handleMouseDown(event: MouseEvent) {
        isDragging = true;
        startX = event.clientX - offsetX;
        startY = event.clientY - offsetY;
    }

    function handleMouseMove(event: MouseEvent) {
        if (isDragging) {
            offsetX = event.clientX - startX;
            offsetY = event.clientY - startY;
        }
    }

    function handleMouseUp() {
        isDragging = false;
    }
</script>

<div class="map-wrapper">
    <!-- Column numbers - scale and translate -->
    <div class="column-numbers" style="transform: translateX({offsetX}px) scale({scale})">
        {#each Array(MAP_SIZE) as _, i}
            <div class="coordinate">{i + 1}</div>
        {/each}
    </div>

    <div class="map-with-rows">
        <!-- Row numbers - scale and translate -->
        <div class="row-numbers" style="transform: translateY({offsetY}px) scale({scale})">
            {#each Array(MAP_SIZE) as _, i}
                <div class="coordinate">{i + 1}</div>
            {/each}
        </div>

        <!-- Map container -->
        <button 
            class="map-container"
            role="application"
            aria-label="Draggable map"
            on:wheel={handleWheel}
            on:mousedown={handleMouseDown}
            on:mousemove={handleMouseMove}
            on:mouseup={handleMouseUp}
            on:mouseleave={handleMouseUp}
            style="transform: translate({offsetX}px, {offsetY}px) scale({scale})"
        >
            {#each tiles as row, y}
                <div class="row">
                    {#each row as tile, x}
                        <div class="tile {tile.type}" data-x={x} data-y={y}></div>
                    {/each}
                </div>
            {/each}
        </button>
    </div>
</div>

<style>
    .map-wrapper {
        position: relative;
        margin: 32px 0 0 32px;
        width: calc(100% - 64px);
        height: calc(100% - 64px);
    }

    .column-numbers {
        display: flex;
        position: absolute;
        top: -32px;
        left: 0;
        gap: 0;
        padding-left: 0;
        z-index: 10;
        transform-origin: 0 0;
    }

    .row-numbers {
        position: absolute;
        left: -32px;
        top: 0;
        display: flex;
        flex-direction: column;
        gap: 0;
        padding-top: 0;
        z-index: 10;
        transform-origin: 0 0;
    }

    .row-numbers .coordinate {
        height: 32px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .coordinate {
        width: 32px;
        height: 24px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 12px;
        color: #666;
        flex-shrink: 0;
    }

    .map-with-rows {
        display: flex;
    }

    .map-container {
        display: flex;
        flex-direction: column;
        transform-origin: 0 0;
        cursor: grab;
        border: none;
        padding: 0;
        background: none;
    }

    .map-container:active {
        cursor: grabbing;
    }

    .row {
        display: flex;
    }

    .tile {
        width: 32px;
        height: 32px;
        border: 1px solid #ccc;
    }

    .grass {
        background-color: #90EE90;
    }
</style>
