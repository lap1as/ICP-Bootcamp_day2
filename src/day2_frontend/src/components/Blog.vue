<template>
    <div>
        <button @click="getEnties">Get Entries</button>
        Hi blog!
        {{ entries }}
        <input v-model="newBlog" type="text">
        <button @click="addEntries">Add Entries</button>
    </div>
</template>

<script>
import { day2_backend } from 'declarations/day2_backend/index';

export default {
    data() {
        return {
            entries: []
        }
    },
    methods: {
        async getEnties() {
            this.entries = await day2_backend.get_entries();
        },
        async addEntries(){
            await day2_backend.add_entry(this.newBlog);
        }
    },
}
</script>