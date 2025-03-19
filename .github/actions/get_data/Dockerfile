# action will be run in python3 container
FROM python:3@sha256:8c55c44b9e81d537f8404d0000b7331863d134db87c1385dd0ec7fefff656495
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
