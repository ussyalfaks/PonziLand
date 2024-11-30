<script lang="ts">    
    let { type } = $props<{
        type: string
    }>()
</script>

<div class="tile {type}"></div>

<style>
    .tile {
        width: 32px;
        height: 32px;
        border: 1px solid #44656b;
    }

    .grass {
        background-image: url('/tiles/grass.jpg');
        background-size: cover;
    }
</style>
