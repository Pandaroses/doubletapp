<script lang="ts">
    import { goto, invalidateAll } from "$app/navigation";
    let isSignup = true;
    let error = "";
  
    async function handleSubmit(e: SubmitEvent) {
      e.preventDefault();
      let data = new URLSearchParams(new FormData(e.target as HTMLFormElement));
      let path = isSignup ? "/api/user/signup" : "/api/user/login";
      const res = await fetch(path, {
        method: "POST",
        headers: {
          "Content-Type": "application/x-www-form-urlencoded",
        },
        body: data,
      });
      if (res?.ok) {
        await invalidateAll();
        goto("/");
      } else {
        switch (res?.status) {
          case 409:
            error = "Username or email already exists";
            break;
          case 401:
            error = "Invalid credentials";
            break;
          case 404:
            error = "not found";
            break;
          default:
            error = "An unknown error occurred";
            break;
        }
      }
    }
  </script>
  
  <div class="min-h-screen w-screen flex items-center justify-center bg-base">
    <div class="w-full max-w-md mx-4 bg-mantle rounded-xl shadow-xl p-8">
      {#if isSignup}
        <div class="flex flex-col gap-8">
          <div class="text-center">
            <h1 class="text-3xl font-medium text-lavender mb-2">Create Account</h1>
          </div>
          <form on:submit={handleSubmit} class="flex flex-col gap-6">
            <div class="flex flex-col gap-4">
              <input
                type="text"
                name="username"
                placeholder="Username"
                class="w-full px-4 py-3 rounded-lg bg-base text-text border border-surface0 focus:border-lavender transition-colors"
              />
              <input
                type="password"
                name="password"
                placeholder="Password"
                class="w-full px-4 py-3 rounded-lg bg-base text-text border border-surface0 focus:border-lavender transition-colors"
              />
            </div>
            {#if error}
              <div class="bg-red/10 border border-red/20 text-red px-4 py-3 rounded-lg text-sm">
                {error}
              </div>
            {/if}
            <button
              type="submit"
              class="w-full px-4 py-3 rounded-lg font-medium bg-lavender text-mantle hover:bg-rosewater transition-colors"
            >
              Sign Up
            </button>
          </form>
          <button
            on:click={() => (isSignup = false)}
            class="text-subtext0 hover:text-text transition-colors pt-2"
          >
            Already have an account? Login here
          </button>
        </div>
      {:else}
        <div class="flex flex-col gap-8">
          <div class="text-center">
            <h1 class="text-3xl font-medium text-lavender mb-2">Welcome Back!</h1>
          </div>
          <form on:submit={handleSubmit} class="flex flex-col gap-6">
            <div class="flex flex-col gap-4">
              <input
                type="username"
                name="username"
                placeholder="Username"
                class="w-full px-4 py-3 rounded-lg bg-base text-text border border-surface0 focus:border-lavender transition-colors"
              />
              <input
                type="password"
                name="password"
                placeholder="Password"
                class="w-full px-4 py-3 rounded-lg bg-base text-text border border-surface0 focus:border-lavender transition-colors"
              />
            </div>
            {#if error}
              <div class="bg-red/10 border border-red/20 text-red px-4 py-3 rounded-lg text-sm">
                {error}
              </div>
            {/if}
            <button
              type="submit"
              class="w-full px-4 py-3 rounded-lg font-medium bg-lavender text-mantle hover:bg-rosewater transition-colors"
            >
              Login
            </button>
          </form>
          <button
            on:click={() => (isSignup = true)}
            class="text-subtext0 hover:text-text transition-colors pt-2"
          >
            Don't have an account? Sign up here
          </button>
        </div>
      {/if}
    </div>
  </div>
