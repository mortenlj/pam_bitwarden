# action will be run in python3 container
FROM python:3@sha256:34306fdf795b9c55c0d258a00e822399cf20572501f92613755ef036283a1814
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
