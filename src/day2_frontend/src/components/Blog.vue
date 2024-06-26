<template>
    <div>
        <h2 class="text-blue-600">All Entries</h2>
        <button @click="getEnties">Refresh</button>
        <div v-for="entry in entries">
            <p>{{ entry }}</p>
        </div>
        <input v-model="newBlog" type="text">
        <button @click="addEntries">Add Entries</button>
    </div>
</template>

<script>
import { day2_backend } from 'declarations/day2_backend/index';

export default {
    data() {
        return {
            entries: [],
            newBlog: ""
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
    async mounted() {
        this.addEntries()
    },
}
</script>