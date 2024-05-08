# Collect Atomic usage issues, including performance issues and thread-safety issues
Crawl project commit messages that contain the keywords "ordering" and "atomic".

## Usage:
To run the program correctly, you should download the chromedriver corresponding to the chrome version.

```sh
cd section-4-2-atomic-safety-issues
# Please provide the URL for the project's commit history page.
python main.py <URL> <CSV_FILE>
```

## Output:

The ```result``` folder generates a csv file of the corresponding result
