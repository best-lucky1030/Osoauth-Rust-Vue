<template>
  <div class="hello">
    <h1> Welcome To Our Medical Firm</h1>
<form>
  <div class="form-group">
    <label for="exampleFormControlInput1">First Name</label>
    <input type="text" class="form-control" v-model="name" id="exampleFormControlInput1" placeholder="Your Name">
  </div>
  <div class="form-group">
    <label for="exampleFormControlInput1">Job ID</label>
    <input type="number" class="form-control" v-model="id" id="exampleFormControlInput1" placeholder="Job ID">
  </div>
  <button type="button" @click="submit" class="btn btn-primary">Enter</button>
  </form>
  </div>
</template>

<script>
import axios from "axios"
export default {
  name: 'Appfront',
    data () {
    return {
      id: '',
      name: ''
    }
    },
      methods: {
    submit() {
      axios({
        url: `http://localhost:3000/jobs/${this.id}`,
        method: "GET",
        headers: { 'user': this.name },
      })
        .then(response => {console.log(response)})
    }
      }
}
</script>

<style scoped>
.hello{
  width:86%;
  margin:auto;
}

.btn{
  width: 40%;
  margin-left: 30%;
  margin-right: 30%;
  margin-bottom: 100px;
  margin-top: 50px;
}
</style>
