import subprocess

def main():
    token_res = subprocess.run(["gh", "auth", "token"], capture_output=True, text=True)
    token = token_res.stdout.strip()
    if not token:
        print("Error: No GitHub token found.")
        return

    remote_url = f"https://{token}@github.com/BamDiggity/typertask-next.git"
    subprocess.run(["git", "remote", "set-url", "origin", remote_url], check=True)
    
    # Push main
    res = subprocess.run(["git", "push", "-u", "origin", "main"])
    if res.returncode == 0:
        print("Successfully pushed TyperTask-Next to GitHub!")
    else:
        print("Push failed with code:", res.returncode)

if __name__ == "__main__":
    main()
