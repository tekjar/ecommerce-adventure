require "kemal"

get "/" do |env|
  # important stuff like clearing session etc.
  env.redirect "/index.html" # redirect to /login page
end

Kemal.run
