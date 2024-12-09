# Git Repository Cleanup Guide

These are step-by-step instructions for removing sensitive files from a Git repository while preserving the commit history.

## Prerequisites

- Git Filter-repo installed (`brew install git-filter-repo` for macOS)

## Step-by-Step Instructions

### 1. Create a Fresh Clone

```bash
# Create a mirror clone of the repository
git clone --mirror git@github.com:username/repository.git repo-filter-repo
cd repo-filter-repo
```

A mirror clone includes all refs and branches, ensuring we can clean the entire repository history. The `--mirror` flag creates a bare repository with all remote branches and tags.

### 2. Remove Sensitive Files

```bash
# Remove specific files from the entire history
git filter-repo --path path/to/sensitive/file.txt --path path/to/another/file --invert-paths
```

`git filter-repo` rewrites history safely. The `--invert-paths` flag keeps everything EXCEPT the specified paths.

### 3. Push Changes Back to Remote

```bash
# Add back the remote origin
git remote add origin git@github.com:username/repository.git

# Force push all changes
git push origin --force --mirror
```

Force pushing is necessary because we've rewritten history. The `--mirror` flag ensures all refs are updated on the remote.

### 4. Create Fresh Local Copy

```bash
# Go to parent directory and clone fresh copy
cd ..
rm -rf old-repository-directory    # Remove old local copy
git clone git@github.com:username/repository.git
```

Gets a fresh copy of the cleaned repository to ensure everything is in sync.

## Important Notes

1. This process rewrites Git history, so all team members will need to re-clone the repository
2. Force pushing is destructive to remote history - make sure you have backups
3. After cleaning up, verify that:
   - The repository still builds/works correctly
   - Sensitive files are completely removed (check GitHub web interface)
   - Commit history is preserved for other files

## Prevention Tips

1. Always use `.gitignore` for sensitive files
2. Review changes before committing (`git diff --staged`)
3. Consider using Git hooks to prevent committing sensitive files