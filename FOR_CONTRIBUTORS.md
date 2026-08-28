Hello!

Ill put some rules down below so the project doesn't get messy.
1. No external libraries
Octo-init should not rely on any external crates.

2. No dynamic allocation.
The "heap" shall not be used for predictability and optimization purposes.

3. Use existing architecture.
Try to use the pre-made helper modules (such as parser) before making your own.

4. Try to make the changes as distant from the main code as possible.
If you have to change code somewhere, its best to try to keep it in the helper modules rather than
putting it into main logic. (prevents hard-coding)

5. Modularity matters.
Always try to make your code as small and as efficient as possible. If its a big chunk 
its better to separate it into its own file.

Thats all for now. the list might grow if needed.
Thanks for contributing!