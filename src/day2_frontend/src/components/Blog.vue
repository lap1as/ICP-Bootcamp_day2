<template>
    <div>
        <h2 class="text-blue-600">Wpisy na bloga:</h2>
        <div class="w-100 flex flex-row-reverse">
            <button @click="getEnties" class="bg-blue-600 rounded text-white p-4">Refresh</button>
        </div>
        <div class="grid mx-6 gap-4 my-4">
            <div v-for="(entry, index) in entries" class="drop-shadow-xl bg-stone-300 p-4">
                <p>{{ index }}</p>
                <p>{{ entry }}</p>               
            </div>
        </div>
        <div class="flex justify-center flex-col">
            <input v-model="newBlog" class="border-2 border-blue-600 p-4" type="text">
            <button  class="bg-blue-600 rounded text-white p-4" @click="addEntries">Add</button>
        </div>
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
        this.getEnties()
    },
}
</script>