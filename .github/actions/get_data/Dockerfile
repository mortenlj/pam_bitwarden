# action will be run in python3 container
FROM python:3@sha256:0aafd87e2438b9db15ffc16e86eed18224c5bc10ab71671f379cae240f3c044e
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
