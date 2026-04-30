from pathlib import Path

class FlowPaths:
    def __init__(self, output_dir: str | Path = "outputs"):
        self.output_dir = Path(output_dir)
        self.logs_dir = self.output_dir / "logs"
        self.csv_dir = self.output_dir / "csv"

        self.create_dirs()

    def create_dirs(self):
        self.output_dir.mkdir(parents=True, exist_ok=True)
        self.logs_dir.mkdir(parents=True, exist_ok=True)
        self.csv_dir.mkdir(parents=True, exist_ok=True)
 
    def csv(self, file_name: str) -> Path:
        return self.csv_dir / f"{file_name}.csv"
