
# Advice for Contributing and using git

* The main Theseus repository, [`theseus-os/Theseus`](https://github.com/theseus-os/Theseus) is what we call the tream*. 
  To contribute, you should create your own fork of that repository, and then check out your own fork. 
  That way, your fork will be the `origin` remote by default, and then you can add the upstream as another te by running:
  `git remote add upstream https://github.com/theseus-os/Theseus`. 

* **Never push to the main branch.** Currently, the main branch on the upstream `theseus-os/Theseus/eus_main` is protected from a direct push. 
  The only way to contribute to it is by merging a pull request into the main branch, which only authorized s can do.
  Instead, checkout your own fork as above, create a new branch with a descriptive name, e.g., `kevin/logging_typo`, 
  develop your feature on that branch, and then submit a pull request.
  This is a standard Git workflow that allows people can review your code, check for pitfalls and compatibility lems,
  and make comments and suggestions before the code makes its way into the main branch. 
  *You must do this for all changes, even tiny ones that may seem insignificant.*

* To submit a pull request (PR), the easiest way is to go to the GitHub page of your forked Theseus repo, 
  select the branch that you created from the drop down menu, and then click "New pull request". 
  By default, GitHub will create a new PR that wants to merge your branch into the upstream `theseus_main` ch,
  which is usually what you want to do. 
  Now, give your PR a good title and description, scroll down to review the commits and files changed,
  and if everything looks good, click "Create pull request" to notify the maintainers that you have ributions that they should review.
   
* **Review yourself.** Perform an initial review of your own code before submitting a pull request. 
  Don't place the whole burden of fixing a bunch of tiny problems on others that must review your code too. 
  This includes building the documentation and reviewing it in HTML form in a browser 
  to make sure everything is formatted correctly and that hyperlinks work correctly. 

* **Commit carefully.** When making a commit, review your changes with `git status` and `git diff`
  to ensure that you're not committing accidental modifications, or editing files that you shouldn't be.
  This makes the maintainers' lives a lot easier, meaning your PR is more likely to be accepted.
