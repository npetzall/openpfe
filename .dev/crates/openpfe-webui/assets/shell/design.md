# Shell — design

The shell wraps all product views: persistent chrome, navigation, and global status.

- **Navigation:** seven peers — see [../README.md](../README.md).
- **Content area:** renders the active view module.
- **Global status:** project context, API connection, errors visible from any view.

Deep-linking between views (e.g. Dashboard → Problem with filter) is desirable; URL scheme deferred until `openpfe-ui` routing is fixed.

Cross-view lifecycle: [../shared/design.md](../shared/design.md).
