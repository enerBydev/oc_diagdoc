
    /// Helper to get files for verification using current options
    fn get_files(&self, data_dir: &PathBuf) -> Vec<PathBuf> {
        Self::get_md_files_with_options(data_dir, self.root_only, &self.exclude)
    }
