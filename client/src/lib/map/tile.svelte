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
        border: 1px solid #ccc;
    }

    .grass {
        background-color: #90EE90;
    }
</style>
