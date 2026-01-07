using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using TauriApp;

var builder = WebAssemblyHostBuilder.CreateDefault(args);

builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddScoped(sp => new HttpClient { BaseAddress = new(builder.HostEnvironment.BaseAddress) });


builder.Services.AddOidcAuthentication(options =>
{
    builder.Configuration.Bind("Cognito", options.ProviderOptions);

    // options.ProviderOptions.DefaultScopes.Add("aws.cognito.signin.user.admin");

    options.UserOptions.NameClaim = "given_name";
    options.UserOptions.RoleClaim = "cognito:groups";

    options.ProviderOptions.PostLogoutRedirectUri = builder.HostEnvironment.BaseAddress;
});

await builder.Build().RunAsync();
